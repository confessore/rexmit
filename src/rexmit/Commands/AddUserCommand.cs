using System;
using MediatR;
using rexmit.Models;

namespace rexmit.Commands;

public class AddUserCommand : IRequest<User>
{
    public ulong Id { get; set; }
    public string? Name { get; set;}
    public string? Email { get; set; }
}
