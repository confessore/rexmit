// rexmit retransmits audio to discord voice channels
//
// Copyright (C) 2024  Steven Confessore
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY, without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

using System;
using Amazon.S3;
using AspNet.Security.OAuth.Discord;
using MediatR;
using Microsoft.AspNetCore.Authentication.Cookies;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Components.Server.Circuits;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.HttpOverrides;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.DependencyInjection.Extensions;
using Microsoft.Extensions.Hosting;
using MudBlazor.Services;
using Npgsql.Replication;
using rexmit;
using rexmit.Behaviors;
using rexmit.Contexts;
using rexmit.Extensions;
using rexmit.Factories;
using rexmit.Models;
using rexmit.Models.Interfaces;
using rexmit.Services;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
builder.Services.AddRazorComponents().AddInteractiveServerComponents();
builder.Services.AddMudServices();
builder.Services.AddControllers();
builder.Services.AddHttpContextAccessor();
builder.Services.ConfigureSignalR();

builder.Services.AddDbContextPool<ApplicationDbContext>(options =>
{
    options.UseNpgsql(
        Environment.GetEnvironmentVariable("POSTGRES_URL")
            ?? builder.Configuration.GetValue<string>("POSTGRES_URL"),
        b => b.MigrationsAssembly("rexmit")
    );
    //options.EnableSensitiveDataLogging();
    options.EnableDetailedErrors();
});
builder.Services.AddDbContextFactory<ApplicationDbContext>(options =>
{
    options.UseNpgsql(
        Environment.GetEnvironmentVariable("POSTGRES_URL")
            ?? builder.Configuration.GetValue<string>("POSTGRES_URL"),
        b => b.MigrationsAssembly("rexmit")
    );
    //options.EnableSensitiveDataLogging();
    options.EnableDetailedErrors();
});
builder.Services.AddDatabaseDeveloperPageExceptionFilter();

builder
    .Services.AddCascadingAuthenticationState()
    .AddAuthentication(options =>
    {
        options.DefaultScheme = CookieAuthenticationDefaults.AuthenticationScheme;
        options.DefaultSignInScheme = CookieAuthenticationDefaults.AuthenticationScheme;
        options.DefaultChallengeScheme = DiscordAuthenticationDefaults.AuthenticationScheme;
    })
    .AddCookie()
    .AddDiscord(options =>
    {
        options.ClientId =
            Environment.GetEnvironmentVariable("DISCORD_CLIENTID")
            ?? builder.Configuration.GetValue<string>("DISCORD_CLIENTID");
        options.ClientSecret =
            Environment.GetEnvironmentVariable("DISCORD_CLIENTSECRET")
            ?? builder.Configuration.GetValue<string>("DISCORD_CLIENTSECRET");
        options.Scope.Add("email");
        options.Scope.Add("guilds");
        options.SaveTokens = true;
    });

builder.Services.AddMediatR(configuration => configuration.RegisterServicesFromAssemblyContaining<Program>());
builder.Services.AddTransient(typeof(IPipelineBehavior<,>), typeof(LoggingPipelineBehavior<,>));
builder.Services.AddSingleton<ISecurityActor, SecurityActor>();
builder.Services.AddSingleton<UserService>();
builder.Services.AddSingleton<DiscordService>();
builder.Services.AddSingleton<FFmpegService>();
builder.Services.AddSingleton<AudioHandlerService>();
builder.Services.ConfigureDiscordBot();

builder.Services.Configure<ForwardedHeadersOptions>(options =>
{
    options.ForwardedHeaders = ForwardedHeaders.XForwardedFor | ForwardedHeaders.XForwardedProto;
});

IAmazonS3 amazonS3Client = new AmazonS3Client(
    Environment.GetEnvironmentVariable("S3_ACCESS_KEY_ID")
        ?? builder.Configuration.GetValue<string>("S3_ACCESS_KEY_ID"),
    Environment.GetEnvironmentVariable("S3_ACCESS_KEY_SECRET")
        ?? builder.Configuration.GetValue<string>("S3_ACCESS_KEY_SECRET"),
    new AmazonS3Config()
    {
        ServiceURL =
            Environment.GetEnvironmentVariable("S3_ENDPOINT")
            ?? builder.Configuration.GetValue<string>("S3_ENDPOINT"),
        ForcePathStyle = true,
        HttpClientFactory = new AmazonS3HttpClientFactory()
    }
);
;

builder.Services.AddSingleton(amazonS3Client);
builder.Services.TryAddEnumerable(ServiceDescriptor.Scoped<CircuitHandler, UserCircuitHandler>());

var app = builder.Build();
await app.MigrateApplicationDbContextAsync();

// Configure the HTTP request pipeline.
if (!app.Environment.IsDevelopment())
{
    app.UseExceptionHandler("/Error", createScopeForErrors: true);
    // The default HSTS value is 30 days. You may want to change this for production scenarios, see https://aka.ms/aspnetcore-hsts.
    app.UseHsts();
}

app.Use(
    async (context, next) =>
    {
        context.Request.Scheme = "https";
        await next.Invoke();
    }
);
app.UseForwardedHeaders();
app.UseCookiePolicy(new CookiePolicyOptions() { Secure = CookieSecurePolicy.Always });

app.UseHttpsRedirection();

app.UseStaticFiles();
app.UseAntiforgery();

app.UseAuthentication();
app.UseAuthorization();

app.MapRazorComponents<rexmit.Components.App>().AddInteractiveServerRenderMode();
app.MapControllers();

await app.StartDiscordBotAsync();

app.Run();
