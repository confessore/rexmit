// Copyright (c) Balanced Solutions Software. All Rights Reserved. Licensed under the MIT license. See LICENSE in the project root for license information.

using Microsoft.AspNetCore.DataProtection.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore;

namespace rexmit.Contexts;

public partial class ApplicationDbContext(DbContextOptions<ApplicationDbContext> options)
    : DbContext(options),
        IDataProtectionKeyContext
{
    public virtual DbSet<DataProtectionKey> DataProtectionKeys { get; set; }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        base.OnModelCreating(modelBuilder);
    }
}
