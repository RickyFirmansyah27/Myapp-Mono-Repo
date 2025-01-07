# Stage 1: Build
FROM openjdk:17-jdk-slim AS build

# Set working directory
WORKDIR /app

# Salin file proyek ke dalam kontainer
COPY . /app

# Instal Maven
RUN apt-get update && apt-get install -y maven

# Build aplikasi dengan Maven (menghasilkan fat JAR)
RUN mvn clean package

# Stage 2: Runtime
FROM openjdk:17-jdk-slim

# Set working directory
WORKDIR /app

# Salin JAR hasil build dari stage sebelumnya
COPY --from=build /app/target/vertx-server-1.0-SNAPSHOT.jar /app/app.jar

# Ekspose port aplikasi
EXPOSE 8106

# Jalankan aplikasi
CMD ["java", "-jar", "app.jar"]
