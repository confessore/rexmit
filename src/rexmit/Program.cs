﻿// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using System;
using Amazon.Runtime;
using System.Net.Http;
using Amazon.S3;
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
using MudBlazor.Services;
using rexmit;
using rexmit.Contexts;
using rexmit.Extensions;
using rexmit.Services;
using rexmit.Factories;

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
    });

builder.Services.AddSingleton<FFmpegService>();
builder.Services.AddSingleton<ThreadHandlerService>();
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
        ServiceURL = Environment.GetEnvironmentVariable("S3_ENDPOINT")
            ?? builder.Configuration.GetValue<string>("S3_ENDPOINT"),
        ForcePathStyle = true,
        HttpClientFactory = new AmazonS3HttpClientFactory()
    }
    ); ;

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
