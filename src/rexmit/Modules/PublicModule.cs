// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using System.Threading.Tasks;
using Discord.Commands;

namespace ShardedClient.Modules;

// Remember to make your module reference the ShardedCommandContext
public class PublicModule : ModuleBase<ShardedCommandContext>
{
    [Command("info")]
    public async Task InfoAsync()
    {
        var msg =
            $@"Hi {Context.User}! There are currently {Context.Client.Shards.Count} shards!
            This guild is being served by shard number {Context.Client.GetShardFor(Context.Guild).ShardId}";
        await ReplyAsync(msg);
    }
}
