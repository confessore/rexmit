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

using System.Threading;
using System.Threading.Tasks;
using MediatR;
using rexmit.Models;
using rexmit.Queries;
using rexmit.Services;

namespace rexmit.QueryHandlers;

public class GetUserByIdQueryHandler(UserService userService) : IRequestHandler<GetUserByIdQuery, User?>
{
    private readonly UserService _userService = userService;

    public async Task<User?> Handle(GetUserByIdQuery request, CancellationToken cancellationToken)
    {
        var user = await _userService.GetUserByIdAsync(request.Id);
        return user;
    }
}
