// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using System;
using System.Threading;
using System.Threading.Tasks;
using Discord.Audio;
using Discord.WebSocket;
using rexmit.Services;

namespace rexmit.Handlers
{
    public class AudioHandler : IDisposable
    {
        private readonly FFmpegService _ffmpegService;

        public AudioHandler(ulong voiceChannelId, FFmpegService ffmpegService, IAudioClient client)
        {
            _ffmpegService = ffmpegService;
            _audioClient = client;
            VoiceChannelId = voiceChannelId;
        }

        public ulong VoiceChannelId { get; }
        public IAudioClient _audioClient;
        public AudioOutStream AudioOutStream { get; set; }

        public async Task PlayAudioAsync(string path)
        {
            await SendFFmpegAsync(_audioClient, path);
        }

        public async Task SendFFmpegAsync(IAudioClient client, string path)
        {
            using var ffmpeg = _ffmpegService.CreateFFmpegStream(path);
            using var output = ffmpeg.StandardOutput.BaseStream;
            if (AudioOutStream is null)
            {
                var discord = client.CreatePCMStream(AudioApplication.Mixed);
                try
                {
                    AudioOutStream = discord;
                    await output.CopyToAsync(discord);
                    Console.WriteLine("copied to output");
                }
                finally
                {
                    //await discord.FlushAsync();
                    Console.WriteLine("flushed");
                }
            }
            else
            {
                await output.CopyToAsync(AudioOutStream);
            }
        }

        public async Task SendFFmpegAsync(
            IAudioClient client,
            string path,
            CancellationToken cancellationToken
        )
        {
            using var ffmpeg = _ffmpegService.CreateFFmpegStream(path);
            using var output = ffmpeg.StandardOutput.BaseStream;
            if (AudioOutStream is null)
            {
                var discord = client.CreatePCMStream(AudioApplication.Mixed);
                try
                {
                    AudioOutStream = discord;
                    await output.CopyToAsync(discord, cancellationToken);
                    Console.WriteLine("copied to output");
                }
                finally
                {
                    //await discord.FlushAsync(cancellationToken);
                    Console.WriteLine("flushed");
                }
            }
            else
            {
                await output.CopyToAsync(AudioOutStream, cancellationToken);
            }
        }

        public async void Dispose()
        {
            await AudioOutStream?.FlushAsync();
            AudioOutStream?.Dispose();
        }
    }
}
