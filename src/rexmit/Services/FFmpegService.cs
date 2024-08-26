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
        var url = $"-c \"ffmpeg -hide_banner -i {videoUrl} -ac 2 -f s16le -ar 48000 pipe:1\"";
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
        var url = $"-hide_banner -loglevel panic -i \"{path}\" -ac 2 -f s16le -ar 48000 pipe:1";
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
