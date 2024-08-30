using System;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Design;
using rexmit.Contexts;

namespace rexmit.Factories;

public class DesignTimeApplicationDbContextFactory : IDesignTimeDbContextFactory<ApplicationDbContext>
{
    public ApplicationDbContext CreateDbContext(string[] args)
    {
        var optionsBuilder = new DbContextOptionsBuilder<ApplicationDbContext>();
        optionsBuilder.UseNpgsql("",
        b => b.MigrationsAssembly("rexmit")
    );

        return new ApplicationDbContext(optionsBuilder.Options);
    }
}
