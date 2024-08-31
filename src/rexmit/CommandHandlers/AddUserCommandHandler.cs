using System.Threading;
using System.Threading.Tasks;
using MediatR;
using rexmit.Commands;
using rexmit.Models;
using rexmit.Services;

namespace rexmit.CommandHandlers;

public class AddUserCommandHandler(UserService userService) : IRequestHandler<AddUserCommand, User>
{
    private readonly UserService _userService = userService;

    public async Task<User> Handle(AddUserCommand request, CancellationToken cancellationToken)
    {
        var user = await _userService.UpsertUserAsync(request.User);
        return user;
    }
}
