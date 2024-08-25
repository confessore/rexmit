// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using rexmit.Handlers;
using System.Collections.ObjectModel;

namespace rexmit.Services
{
    public class AudioHandlerService
    {
        public AudioHandlerService()
        {
            AudioHandlers = [];
        }

        public Collection<AudioHandler> AudioHandlers { get; set; }
    }
}
