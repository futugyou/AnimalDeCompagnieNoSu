using Microsoft.EntityFrameworkCore;
using Volo.Abp.Data;
using Volo.Abp.EntityFrameworkCore;

namespace Adoption.Infrastruct.Data.DbMigrations
{ 
    [ConnectionStringName("Default")]
    public class AdoptionMigrationsDbContext : AbpDbContext<AdoptionMigrationsDbContext>
    {
        public AdoptionMigrationsDbContext(
            DbContextOptions<AdoptionMigrationsDbContext> options)
            : base(options)
        {
        }

        protected override void OnModelCreating(ModelBuilder modelBuilder)
        {
            base.OnModelCreating(modelBuilder);

            modelBuilder.ConfigureAdoption();
        }
    }
}