// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using System.Threading;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Components.Authorization;
using Microsoft.AspNetCore.Components.Server.Circuits;

namespace rexmit;

public sealed class UserCircuitHandler(AuthenticationStateProvider authenticationStateProvider)
    : CircuitHandler
{
    public override async Task OnConnectionUpAsync(
        Circuit circuit,
        CancellationToken cancellationToken
    )
    {
        _ = await authenticationStateProvider.GetAuthenticationStateAsync();
    }
}
