// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using System;
using AspNet.Security.OAuth.Discord;
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
using rexmit;
using rexmit.Components;
using rexmit.Contexts;
using rexmit.Extensions;
using rexmit.Services;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
builder.Services.AddRazorComponents().AddInteractiveServerComponents();
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
    });

builder.Services.AddSingleton<FFmpegService>();
builder.Services.AddSingleton<ThreadHandlerService>();
builder.Services.ConfigureDiscordBot();

builder.Services.Configure<ForwardedHeadersOptions>(options =>
{
    options.ForwardedHeaders = ForwardedHeaders.XForwardedFor | ForwardedHeaders.XForwardedProto;
});

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

app.MapRazorComponents<App>().AddInteractiveServerRenderMode();
app.MapControllers();

await app.StartDiscordBotAsync();

app.Run();
