using System.Threading;
using System.Threading.Tasks;
using rexmit.Models;
using rexmit.Models.Interfaces;
using rexmit.Queries;

namespace rexmit.QueryHandlers;

public class GetUserByIdQueryHandler : IRequestHandler<GetUserByIdQuery, User>
{
    public Task<User> Handle(GetUserByIdQuery request, CancellationToken cancellationToken)
    {
        User user = new(); /* add logic to get a user from the db */
        return Task.FromResult(user);
    }
}
