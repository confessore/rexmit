using Discord;
using rexmit.Models.Abstractions;

namespace rexmit.Models;

public class User : Entity<ulong>
{
    public string? Name { get; set; }
    public string? Email { get; set; }
}
