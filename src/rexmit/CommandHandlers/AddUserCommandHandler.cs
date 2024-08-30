using System.Threading;
using System.Threading.Tasks;
using rexmit.Commands;
using rexmit.Models.Interfaces;

namespace rexmit.CommandHandlers;

public class AddUserCommandHandler : IRequestHandler<AddUserCommand, ulong>
{
    public Task<ulong> Handle(AddUserCommand request, CancellationToken cancellationToken)
    {
        ulong newUserId = 0; /* add logic to add a user to db */
        return Task.FromResult(newUserId);
    }
}
