using System.Threading;
using System.Threading.Tasks;
using MediatR;
using rexmit.Commands;
using rexmit.Models;
using rexmit.Services;

namespace rexmit.CommandHandlers;

public class AddUserCommandHandler(UserService userService) : IRequestHandler<AddUserCommand, ulong>
{
    private readonly UserService _userService = userService;

    public async Task<ulong> Handle(AddUserCommand request, CancellationToken cancellationToken)
    {
        var user = new User() { Id = request.Id, Name = request.Name};
        await _userService.UpsertUserAsync(user);
        return user.Id;
    }
}
