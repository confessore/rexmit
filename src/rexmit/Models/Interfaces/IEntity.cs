using System;

namespace rexmit.Models.Interfaces;

public interface IEntity<TId>
    where TId : IEquatable<TId>
{
    TId Id { get; set; }
    DateTime? CreatedAt { get; set; }
    string? CreatedBy { get; set; }
    DateTime? UpdatedAt { get; set; }
    string? UpdatedBy { get; set; }
}
