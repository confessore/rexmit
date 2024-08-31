using System;
using Discord;

namespace rexmit.Models.Abstractions;

public abstract class Entity<TId> : IEntity<TId>
    where TId : IEquatable<TId>
{
    public TId Id { get; set; } = default!;
    public DateTime? CreatedAt { get; set; }
    public ulong? CreatedBy { get; set; }
    public DateTime? UpdatedAt { get; set; }
    public ulong? UpdatedBy { get; set; }
}
