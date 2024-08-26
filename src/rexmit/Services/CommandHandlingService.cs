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
using System.Reflection;
using System.Threading.Tasks;
using Discord;
using Discord.Commands;
using Discord.WebSocket;
using Microsoft.Extensions.DependencyInjection;

namespace rexmit.Services;

public class CommandHandlingService
{
    private readonly CommandService _commands;
    private readonly DiscordShardedClient _discord;
    private readonly IServiceProvider _services;

    public CommandHandlingService(IServiceProvider services)
    {
        _commands = services.GetRequiredService<CommandService>();
        _discord = services.GetRequiredService<DiscordShardedClient>();
        _services = services;

        _commands.CommandExecuted += CommandExecutedAsync;
        _commands.Log += LogAsync;
        _discord.MessageReceived += MessageReceivedAsync;
    }

    public async Task InitializeAsync()
    {
        await _commands.AddModulesAsync(Assembly.GetEntryAssembly(), _services);
    }

    public async Task MessageReceivedAsync(SocketMessage rawMessage)
    {
        // Ignore system messages, or messages from other bots
        if (rawMessage is not SocketUserMessage message)
        {
            return;
        }

        if (message.Source != MessageSource.User)
        {
            return;
        }

        // This value holds the offset where the prefix ends
        var argPos = 0;
        if (!message.HasMentionPrefix(_discord.CurrentUser, ref argPos))
        {
            return;
        }

        // A new kind of command context, ShardedCommandContext can be utilized with the commands framework
        var context = new ShardedCommandContext(_discord, message);
        await _commands.ExecuteAsync(context, argPos, _services);
    }

    public static async Task CommandExecutedAsync(
        Optional<CommandInfo> command,
        ICommandContext context,
        Discord.Commands.IResult result
    )
    {
        // command is unspecified when there was a search failure (command not found); we don't care about these errors
        if (!command.IsSpecified)
        {
            return;
        }

        // the command was successful, we don't care about this result, unless we want to log that a command succeeded.
        if (result.IsSuccess)
        {
            return;
        }

        // the command failed, let's notify the user that something happened.
        await context.Channel.SendMessageAsync($"error: {result}");
    }

    private Task LogAsync(LogMessage log)
    {
        Console.WriteLine(log.ToString());

        return Task.CompletedTask;
    }
}
