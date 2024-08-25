// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using System.Threading.Tasks;
using Discord.Audio;
using rexmit.Services;

namespace rexmit.Handlers
{
    public class AudioHandler
    {
        private readonly FFmpegService _ffmpegService;

        public AudioHandler(
            ulong voiceChannelId,
            FFmpegService ffmpegService,
            IAudioClient client
        )
        {
            _ffmpegService = ffmpegService;
            _audioClient = client;
            VoiceChannelId = voiceChannelId;
        }
        public ulong VoiceChannelId { get; }
        public IAudioClient _audioClient;

        public async Task PlayAudioAsync(string path)
        {
            await _ffmpegService.SendFFmpegAsync(_audioClient, path);
        }
    }
}
