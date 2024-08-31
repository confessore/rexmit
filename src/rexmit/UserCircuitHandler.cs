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
using System.ComponentModel.DataAnnotations;
using System.Diagnostics;
using System.Linq;
using System.Security.Claims;
using System.Threading;
using System.Threading.Tasks;
using MediatR;
using Microsoft.AspNetCore.Components.Authorization;
using Microsoft.AspNetCore.Components.Server.Circuits;
using rexmit.Commands;
using rexmit.Models;
using rexmit.Models.Interfaces;
using rexmit.Queries;

namespace rexmit;

public sealed class UserCircuitHandler(AuthenticationStateProvider authenticationStateProvider, ISecurityActor securityActor, IMediator mediator)
    : CircuitHandler
{
    private readonly AuthenticationStateProvider _authenticationStateProvider = authenticationStateProvider;
    private readonly ISecurityActor _securityActor = securityActor;
    private readonly IMediator _mediator = mediator;

    public override async Task OnConnectionUpAsync(
        Circuit circuit,
        CancellationToken cancellationToken
    )
    {
        var authenticationState = await _authenticationStateProvider.GetAuthenticationStateAsync();
        var nameIdentifier = authenticationState.User.Claims.FirstOrDefault(x => x.Type == ClaimTypes.NameIdentifier);
        if (nameIdentifier is not null)
        {
            var id = Convert.ToUInt64(nameIdentifier.Value); 
            var user = await _mediator.Send(new GetUserByIdQuery() { Id = id }, cancellationToken);
            if (user is not null)
            {
                _securityActor.DiscordId = user.Id;
                _securityActor.Name = user.Name;
            }
            else
            {
                var name = authenticationState.User.Claims.FirstOrDefault(x => x.Type == ClaimTypes.Name);
                if (name is not null)
                {
                    var email = authenticationState.User.Claims.FirstOrDefault(x => x.Type == ClaimTypes.Email);
                    if (email is not null)
                    {
                        await _mediator.Send(new AddUserCommand() { Id = id, Name = name.Value, Email = email.Value }, cancellationToken);
                    }
                }
            }
        }
    }

}
