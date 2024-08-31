using System.Security.Cryptography.X509Certificates;
using System.Threading.Tasks;
using Microsoft.EntityFrameworkCore;
using rexmit.Contexts;
using rexmit.Models;

namespace rexmit.Services;

public class UserService(IDbContextFactory<ApplicationDbContext> applicationDbContextFactory)
{
    private readonly IDbContextFactory<ApplicationDbContext> _applicationDbContextFactory = applicationDbContextFactory;

    public async Task<User?> GetUserByIdAsync(ApplicationDbContext context, ulong id) =>
        await context.Users.FirstOrDefaultAsync(user => user.Id == id);

    public async Task<User?> GetUserByIdAsync(ulong id)
    {
        using var context = await _applicationDbContextFactory.CreateDbContextAsync();
        return await GetUserByIdAsync(context, id);
    }

    public async Task<User> UpsertUserAsync(ApplicationDbContext context, User user)
    {
        var entity = await context.Users.FirstOrDefaultAsync(user => user.Id == user.Id);
        if (entity is null)
        {
            await context.Users.AddAsync(user);
        }
        else if (entity is not null)
        {
            entity = user;
        }

        await context.SaveChangesAsync();
        return entity;
    }


    public async Task<User> UpsertUserAsync(User user)
    {
        using var context = await _applicationDbContextFactory.CreateDbContextAsync();
        return await UpsertUserAsync(context, user);
    }
}
