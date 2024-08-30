using rexmit.Models;
using rexmit.Models.Interfaces;

namespace rexmit.Queries;

public class GetUserByIdQuery : IRequest<User>
{
    public ulong Id { get; set; }
}
