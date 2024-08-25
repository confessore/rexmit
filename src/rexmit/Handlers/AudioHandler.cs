// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Discord.Audio;
using rexmit.Services;

namespace rexmit.Handlers;

public class AudioHandler
{
    private readonly FFmpegService _ffmpegService;

    public AudioHandler(ulong voiceChannelId, FFmpegService ffmpegService, IAudioClient client)
    {
        _ffmpegService = ffmpegService;
        _audioClient = client;
        VoiceChannelId = voiceChannelId;
    }

    public ulong VoiceChannelId { get; }
    private bool _started;
    public IAudioClient _audioClient;
    private Thread _thread;
    private CancellationTokenSource _cancellationTokenSource;
    public event Action OnTrackStart;
    public event Action OnTrackEnd;
    private List<string> _queue;
    public AudioOutStream AudioOutStream { get; set; }

    private async Task PlayCurlAsync(string path)
    {
        await SendCurlAsync(_audioClient, path);
    }

    private async Task PlayAudioAsync(string path, CancellationToken cancellationToken)
    {
        await SendFFmpegAsync(_audioClient, path, cancellationToken);
    }

    private async Task SendCurlAsync(IAudioClient client, string path)
    {
        using var ffmpeg = _ffmpegService.CreateCurlStream(path);
        using var output = ffmpeg.StandardOutput.BaseStream;
        if (AudioOutStream is null)
        {
            var discord = client.CreatePCMStream(AudioApplication.Mixed);
            try
            {
                AudioOutStream = discord;
                await output.CopyToAsync(AudioOutStream);
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

    private async Task SendFFmpegAsync(IAudioClient client, string path)
    {
        using var ffmpeg = _ffmpegService.CreateFFmpegStream(path);
        using var output = ffmpeg.StandardOutput.BaseStream;
        if (AudioOutStream is null)
        {
            var discord = client.CreatePCMStream(AudioApplication.Mixed);
            try
            {
                AudioOutStream = discord;
                await output.CopyToAsync(AudioOutStream);
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

    private async Task SendFFmpegAsync(
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
                await output.CopyToAsync(AudioOutStream, cancellationToken);
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

    public void Queue(string url)
    {
        _queue ??= [];
        _queue.Add(url);
        StartThread();
    }

    public void Dequeue()
    {
        _queue ??= [];
        _queue.RemoveAt(_queue.Count - 1);
        StartThread();
    }

    public void Skip()
    {
        _queue ??= [];
        _queue.RemoveAt(0);
        _cancellationTokenSource.Cancel();
        _started = false;
        StartThread();
    }

    public void Insert(string url)
    {
        _queue ??= [];
        _queue.Insert(1, url);
        StartThread();
    }

    // Start the thread and store its reference
    public void StartThread()
    {
        if (!_started)
        {
            _started = true;
            _cancellationTokenSource = new CancellationTokenSource();
            _thread = new Thread(async () => await ThreadWorkAsync(_cancellationTokenSource.Token));
            _thread.Start();
            Console.WriteLine("Thread started.");
        }
    }

    // Stop the thread using the stored reference
    public void StopThread()
    {
        if (_cancellationTokenSource != null)
        {
            _started = false;
            _cancellationTokenSource.Cancel();
            _thread.Join(); // Wait for the thread to finish
            Console.WriteLine("Thread stopped.");
        }
    }

    // Retrieve the thread reference
    public Thread GetThread()
    {
        return _thread;
    }

    // The work that the thread will perform
    private async Task ThreadWorkAsync(CancellationToken token)
    {
        try
        {
            while (!token.IsCancellationRequested)
            {
                Console.WriteLine("Thread is working...");
                if (_queue.Count > 0)
                {
                    OnTrackStart?.Invoke();
                    //await _interactionModule.Context.Channel.SendMessageAsync($"Now playing {_queue[0]}");
                    await PlayCurlAsync(_queue[0]);
                    Dequeue();
                    OnTrackEnd?.Invoke();
                }
                else
                {
                    StopThread();
                }
            }
        }
        catch (OperationCanceledException)
        {
            Console.WriteLine("Thread is stopping due to cancellation.");
        }
        finally
        {
            if (_queue.Count == 0)
            {
                //StopThread();
            }
        }
    }
}
