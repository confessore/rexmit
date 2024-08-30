using rexmit.Models.Interfaces;

namespace rexmit.Commands;

public class AddUserCommand : IRequest<ulong>
{
    public string? Name { get; set;}
}
