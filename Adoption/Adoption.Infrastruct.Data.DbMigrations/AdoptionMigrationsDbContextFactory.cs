using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Design;
using Microsoft.Extensions.Configuration;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Adoption.Infrastruct.Data.DbMigrations
{
    /* This class is needed for EF Core console commands
     * (like Add-Migration and Update-Database commands) */
    public class AdoptionMigrationsDbContextFactory : IDesignTimeDbContextFactory<AdoptionMigrationsDbContext>
    {
        public AdoptionMigrationsDbContext CreateDbContext(string[] args)
        {
            var configuration = BuildConfiguration();
            var serverVersion = new MySqlServerVersion(new Version(configuration["DefaultMysqlVersion"]));
            var builder = new DbContextOptionsBuilder<AdoptionMigrationsDbContext>()
                .UseMySql(configuration.GetConnectionString("Default"), serverVersion);

            return new AdoptionMigrationsDbContext(builder.Options);
        }

        private static IConfigurationRoot BuildConfiguration()
        {
            var builder = new ConfigurationBuilder()
                .SetBasePath(Path.Combine(Directory.GetCurrentDirectory(), "../Adoption.DbMigrator/"))
                .AddJsonFile("appsettings.json", optional: false);

            return builder.Build();
        }
    }
}
