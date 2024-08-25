// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using System;
using System.Collections.Concurrent;
using System.Diagnostics;
using System.Threading;
using System.Threading.Tasks;
using Discord.Audio;

namespace rexmit.Services;

public partial class FFmpegService
{
    private readonly ConcurrentDictionary<ulong, AudioOutStream> cd =
        new ConcurrentDictionary<ulong, AudioOutStream>();

    public Process CreateYTDLPStream(string videoUrl)
    {
        var url =
            $"-c \"yt-dlp -o - {videoUrl} | ffmpeg -hide_banner -loglevel panic -i pipe:0 -ac 2 -f s16le -ar 48000 pipe:1\"";
        Console.WriteLine(url);
        var info = new ProcessStartInfo()
        {
            FileName = "/bin/bash",
            Arguments = url,
            UseShellExecute = false,
            RedirectStandardOutput = true
        };
        return Process.Start(info) ?? default!;
    }

    public Process CreateCurlStream(string videoUrl)
    {
        var url =
            $"-c \"ffmpeg -hide_banner -i {videoUrl} -ac 2 -f s16le -ar 48000 pipe:1\"";
        Console.WriteLine(url);
        var info = new ProcessStartInfo()
        {
            FileName = "/bin/bash",
            Arguments = url,
            UseShellExecute = false,
            RedirectStandardOutput = true
        };
        return Process.Start(info) ?? default!;
    }

    public Process CreateFFmpegStream(string path)
    {
        var url =
            $"-hide_banner -loglevel panic -i \"{path}\" -ac 2 -f s16le -ar 48000 pipe:1";
        Console.WriteLine(url);
        var info = new ProcessStartInfo()
        {
            FileName = "ffmpeg",
            Arguments = url,
            UseShellExecute = false,
            RedirectStandardOutput = true
        };
        return Process.Start(info) ?? default!;
    }

    public async Task SendFFmpegAsync(IAudioClient client, string path)
    {
        // Create FFmpeg using the previous example
        using var ffmpeg = CreateFFmpegStream(path);
        using var output = ffmpeg.StandardOutput.BaseStream;
        using var discord = client.CreatePCMStream(AudioApplication.Mixed);
        try
        {
            await output.CopyToAsync(discord);
            Console.WriteLine("copied to output");
        }
        finally
        {
            //await discord.FlushAsync();
            Console.WriteLine("flushed");
        }
    }

    public async Task SendFFmpegAsync(
        IAudioClient client,
        string path,
        CancellationToken cancellationToken
    )
    {
        // Create FFmpeg using the previous example
        using var ffmpeg = CreateFFmpegStream(path);
        using var output = ffmpeg.StandardOutput.BaseStream;
        using var discord = client.CreatePCMStream(AudioApplication.Mixed);
        try
        {
            await output.CopyToAsync(discord, cancellationToken);
            Console.WriteLine("copied to output");
        }
        finally
        {
            //await discord.FlushAsync(cancellationToken);
            Console.WriteLine("flushed");
        }
    }

    public async Task SendYTLDPAsync(IAudioClient client, string path)
    {
        // Create FFmpeg using the previous example
        using var ffmpeg = CreateYTDLPStream(path);
        using var output = ffmpeg.StandardOutput.BaseStream;
        using var discord = client.CreatePCMStream(AudioApplication.Mixed);
        try
        {
            await output.CopyToAsync(discord);
            Console.WriteLine("copied to output");
        }
        finally
        {
            await discord.FlushAsync();
            Console.WriteLine("flushed");
        }
    }

    public async Task SendCurlAsync(IAudioClient client, string path)
    {
        // Create FFmpeg using the previous example
        using var ffmpeg = CreateCurlStream(path);
        using var output = ffmpeg.StandardOutput.BaseStream;
        using var discord = client.CreatePCMStream(AudioApplication.Mixed);
        try
        {
            await output.CopyToAsync(discord);
            Console.WriteLine("copied to output");
        }
        finally
        {
            await discord.FlushAsync();
            Console.WriteLine("flushed");
        }
    }

    public async Task SendYTDLPAsync(
        IAudioClient client,
        string path,
        CancellationToken cancellationToken
    )
    {
        // Create FFmpeg using the previous example
        using var ffmpeg = CreateYTDLPStream(path);
        using var output = ffmpeg.StandardOutput.BaseStream;
        using var discord = client.CreatePCMStream(AudioApplication.Mixed);
        try
        {
            await output.CopyToAsync(discord, cancellationToken);
            Console.WriteLine("copied to output");
        }
        finally
        {
            await discord.FlushAsync(cancellationToken);
            Console.WriteLine("flushed");
        }
    }
}
