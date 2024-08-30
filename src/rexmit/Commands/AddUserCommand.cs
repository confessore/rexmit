using System;
using MediatR;

namespace rexmit.Commands;

public class AddUserCommand : IRequest<ulong>
{
    public ulong Id { get; set; }
    public string? Name { get; set;}
}
