// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using System.Collections.Generic;
using rexmit.Handlers;

namespace rexmit.Services
{
    public sealed class ThreadHandlerService
    {
        public ThreadHandlerService()
        {
            ThreadHandlers = [];
        }

        public List<ThreadHandler> ThreadHandlers { get; set; }
    }
}
