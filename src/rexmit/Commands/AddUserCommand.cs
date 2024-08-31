using MediatR;
using rexmit.Models;

namespace rexmit.Commands;

public record AddUserCommand(User User) : IRequest<User>
{

}
