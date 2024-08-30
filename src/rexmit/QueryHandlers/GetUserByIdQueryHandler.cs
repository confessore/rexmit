using System;
using System.Linq;
using System.Security.Claims;
using System.Threading;
using System.Threading.Tasks;
using MediatR;
using Microsoft.AspNetCore.Components.Authorization;
using rexmit.Models;
using rexmit.Models.Interfaces;
using rexmit.Queries;
using rexmit.Services;

namespace rexmit.QueryHandlers;

public class GetUserByIdQueryHandler(UserService userService) : IRequestHandler<GetUserByIdQuery, User>
{
    private readonly UserService _userService = userService;

    public async Task<User> Handle(GetUserByIdQuery request, CancellationToken cancellationToken)
    {
        var user = await _userService.GetUserByIdAsync(request.Id) ?? new() { Id = 0, Name = "Anonymous" };
        return user;
    }
}
