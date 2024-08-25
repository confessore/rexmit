// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Discord.Audio;
using rexmit.Modules;
using rexmit.Services;

namespace rexmit.Handlers;

public class ThreadHandler
{
    private readonly FFmpegService _ffmpegService;

    public ThreadHandler(
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
    private bool _started;
    public IAudioClient _audioClient;
    private Thread _thread;
    private CancellationTokenSource _cancellationTokenSource;
    public event Action OnTrackStart;
    public event Action OnTrackEnd;
    private List<string> _queue;

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

                OnTrackStart?.Invoke();
                //await _interactionModule.Context.Channel.SendMessageAsync($"Now playing {_queue[0]}");
                await _ffmpegService.SendFFmpegAsync(_audioClient, _queue[0], token);
                if (_queue.Count > 0)
                {
                    _queue.RemoveAt(0);
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
