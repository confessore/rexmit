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
        var id = authenticationState.User.Claims.FirstOrDefault(x => x.Type == ClaimTypes.NameIdentifier);
        var name = authenticationState.User.Claims.FirstOrDefault(x => x.Type == ClaimTypes.Name);
        var email = authenticationState.User.Claims.FirstOrDefault(x => x.Type == ClaimTypes.Email);
        if (id is not null && name is not null && email is not null)
        {
            var user = await _mediator.Send(new GetUserByIdQuery(Convert.ToUInt64(id.Value)), cancellationToken);
            user ??= new User()
            {
                Id = Convert.ToUInt64(id.Value),
                CreatedAt = DateTime.UtcNow,
                CreatedBy = Convert.ToUInt64(id.Value)
            };
            user.Name = name.Value;
            user.Email = email.Value;
            user.UpdatedAt = DateTime.UtcNow;
            user.UpdatedBy = Convert.ToUInt64(id.Value);
            user = await _mediator.Send(new AddUserCommand(user), cancellationToken);
            _securityActor.DiscordId = user.Id;
            _securityActor.Name = user.Name;
            _securityActor.Email = user.Email;
        }
    }
}
