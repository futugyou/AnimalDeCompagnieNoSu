###### 1. Integrate abp vNext with empty aspcnetcore web api project. Current Abp Version 4.3.0
1. Create an empty web api project, called Adoption.Host.
2. Install-Package Volo.Abp.AspNetCore/Volo.Abp.Swashbuckle.
3. Create AdoptionHostModule impl AbpModule, add AbpAspNetCoreModule and AbpSwashbuckleModule DependsOnAttribute.
4. Override ConfigureServices and OnApplicationInitialization
5. Move code from startup to AdoptionHostModule, fill ConfigureServices and Configure in startup

###### 2. Add Domain to sln
1. Create an lib project, called Adoption.Domain.
2. Install-Package Volo.Abp.AspNetCore/Volo.Abp.Ddd.Domain.
3. Create AdoptionDomainModule impl AbpModule, add AbpDddDomainModule DependsOnAttribute.
4. Create a Domain (e.g Animals) impl Entity<T> or AggregateRoot<T> or others.

###### 3. Add Infrastruct project (e.g.  dbcontext) to sln
1. Create an lib project, called Adoption.Infrastruct.Data.
2. Install-Package Microsoft.EntityFrameworkCore.Design to Adoption.Host. (Use ef5.0.5, becase ef6.0-preview have some problem now. 2021/4/26)
3. Install-Package Microsoft.EntityFrameworkCore.Tools to Adoption.Infrastruct.Data.
4. Install-Package Volo.Abp.AspNetCore/Volo.Abp.EntityFrameworkCore/Volo.Abp.EntityFrameworkCore.MySQL(or other) to Adoption.Infrastruct.Data.
5. Create AdoptionInfrastructDataModule impl AbpModule, add AdoptionDomainModule and AbpEntityFrameworkCoreModule DependsOnAttribute.
6. Create DbContext impl AbpDbContext<DbContext> , add DbSet<Domain>, and you can override base methods if you need.
7. AddAbpDbContext<DbContext> and Configure<AbpDbContextOptions> in AdoptionInfrastructDataModule ConfigureServices method.

###### 4. Add Application Contracts to sln
1. Create an lib project, called Adoption.Application.Contracts.
2. Install-Package Volo.Abp.Ddd.Application.Contracts.
3. Create Dto (e.g AnimalDto) impl EntityDto<T>.
4. Create AdoptionApplicationContractsMdoule impl AbpModule.
5. Create 'Service' Interface inherit IApplicationService

###### 5. Add Application to sln
1. Create an lib project, called Adoption.Application.
2. Install-Package Volo.Abp.AspNetCore/Volo.Abp.AutoMapper/Volo.Abp.Ddd.Application.
3. Create AutoMapperProfile impl Profile.
4. Create Serviceimpl inherit ApplicationService and impl 'Service'.
5. Create AdoptionApplicationModule impl AbpModule, add  AbpDddApplicationModule/AbpAutoMapperModule/AdoptionInfrastructDataModule/AdoptionApplicationContractsMdoule DependsOnAttribute.
6. Configure<AbpAutoMapperOptions> in order to load all AutoMapperProfile in this module.
7. Add AdoptionApplicationModule DependsOnAttribute to AdoptionHostModule.

###### 6. Localization and VirtualFiles
1. Add localization file to project.
2. Add typeof(AbpLocalizationModule) (include AbpVirtualFileSystemModule) to Module.
3. Configure AbpVirtualFileSystemOptions/AbpLocalizationOptions/AbpExceptionLocalizationOptions.
4. Edit csproj like this:
```
  <PropertyGroup>
    <TargetFramework>net6.0</TargetFramework>
    <GenerateEmbeddedFilesManifest>true</GenerateEmbeddedFilesManifest>
  </PropertyGroup>
  <ItemGroup>
    <EmbeddedResource Include="Localization\**\*.json" />
    <Content Remove="Localization\**\*.json" />
    <PackageReference Include="Microsoft.Extensions.FileProviders.Embedded" Version="5.0.5" />
  </ItemGroup>
```
5. Add Configure AbpLocalizationOptions and app.UseAbpRequestLocalization()/app.UseStaticFiles() to Host Module.

###### 7. FluentValidation
1. Install-Package Volo.Abp.FluentValidation to Adoption.Application.Contracts.
2. Add Validator impl AbstractValidator<T> ,then abp runtime will identify Validator automatically.
3. AdoptionApplicationContractsMdoule MUST BE ADD AS DependsOn to Adoption.Application.

###### 8. Background Jobs
1. Install-Package Volo.Abp.BackgroundJobs.Abstractions and Volo.Abp.BackgroundJobs.RabbitMQ to Adoption.Application.
2. Add AbpBackgroundJobsRabbitMqModule DependsOnAttribute to Module.
3. (option) Configure AbpRabbitMqOptions. If not, abp will use appsetting.json as default.(job will use Queue directly)
4. impl your own job logic impl AsyncBackgroundJob<T> and ITransientDependency, T is job parameter. Then abp runtime will identify Job automatically. 

###### 9. Distributed Event Bus (rabbitmq)
1. Install-Package Volo.Abp.EventBus.RabbitMQ to Adoption.Application.
2. Add AbpEventBusRabbitMqModule DependsOnAttribute to Module.
3. (option) Configure AbpRabbitMqOptions. Same as Background Jobs.(ClientName is Queuename)
4. Create Event Transfer Objects(Eto) With especial EventNameAttribute. It will be used as rabbitmq Routing key.
5. Use IDistributedEventBus.PublishAsync to publish message.
6. impl IDistributedEventHandler<T> will add a subscribe to T. Then abp runtime will identify subscribe automatically.

###### 00. Environment
1. mariadb
```
docker run -p 127.0.0.1:3306:3306  --name  mariadb -e MYSQL_ROOT_PASSWORD=123456 -d mariadb
```

2. mirage database
```
add-migration init -Context AdoptionDbContext
update-database -Context AdoptionDbContext
```

3. debug in docker 
https://code.visualstudio.com/docs/containers/quickstart-aspnet-core

4. rabbit in docker
```
docker run -itd --hostname Terraform --name AbpEventMQ -e RABBITMQ_DEFAULT_USER=user -e RABBITMQ_DEFAULT_PASS=password  -p 15672:15672 -p 5672:5672 rabbitmq:3-management
```

5. kafka in docker 
```
docker run -d --name zookeeper -p 2181:2181 -t wurstmeister/zookeeper
docker run -d --name kafka -p 9092:9092 -e KAFKA_BROKER_ID=0 -e KAFKA_ZOOKEEPER_CONNECT=10.0.0.4:2181 -e KAFKA_ADVERTISED_LISTENERS=PLAINTEXT://10.0.0.4:9092 -e KAFKA_LISTENERS=PLAINTEXT://0.0.0.0:9092 -t wurstmeister/kafka

docker exec -it 245317b191e2  /bin/bash
cd opt/kafka/bin
kafka-topics.sh --create --zookeeper 10.0.0.4:2181 --replication-factor 1 --partitions 1 --topic mykafka
kafka-console-producer.sh --broker-list localhost:9092 --topic mykafka
kafka-console-consumer.sh --bootstrap-server  localhost:9092 --topic mykafka --from-beginning
```