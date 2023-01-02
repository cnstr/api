-- CreateEnum
CREATE TYPE "ManifestType" AS ENUM ('UnsafeRepo', 'ScamJailbreak', 'ScamUnlock');

-- CreateTable
CREATE TABLE "Repository" (
    "slug" TEXT NOT NULL,
    "aliases" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "tier" INTEGER NOT NULL,
    "packageCount" INTEGER NOT NULL,
    "sections" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "uri" TEXT NOT NULL,
    "suite" TEXT NOT NULL DEFAULT './',
    "component" TEXT,
    "name" TEXT,
    "version" TEXT,
    "description" TEXT,
    "date" TIMESTAMP(3),
    "paymentGateway" TEXT,
    "sileoEndpoint" TEXT,
    "isPruned" BOOLEAN NOT NULL DEFAULT false,
    "originId" TEXT NOT NULL,

    CONSTRAINT "Repository_pkey" PRIMARY KEY ("slug")
);

-- CreateTable
CREATE TABLE "Origin" (
    "uuid" TEXT NOT NULL,
    "hostname" TEXT NOT NULL,
    "releasePath" TEXT NOT NULL,
    "packagesPath" TEXT NOT NULL,
    "lastUpdated" TIMESTAMP(3) NOT NULL,
    "hasInRelease" BOOLEAN NOT NULL,
    "hasReleaseGPG" BOOLEAN NOT NULL,
    "supportsPaymentV1" BOOLEAN NOT NULL,
    "supportsPaymentV2" BOOLEAN NOT NULL,
    "usesHTTPS" BOOLEAN NOT NULL,

    CONSTRAINT "Origin_pkey" PRIMARY KEY ("uuid")
);

-- CreateTable
CREATE TABLE "Package" (
    "uuid" TEXT NOT NULL,
    "package" TEXT NOT NULL,
    "isCurrent" BOOLEAN NOT NULL,
    "isPruned" BOOLEAN NOT NULL DEFAULT false,
    "repositoryTier" INTEGER NOT NULL,
    "repositorySlug" TEXT NOT NULL,
    "price" TEXT NOT NULL,
    "version" TEXT NOT NULL,
    "architecture" TEXT NOT NULL,
    "filename" TEXT NOT NULL,
    "size" INTEGER NOT NULL,
    "sha256" TEXT,
    "name" TEXT,
    "description" TEXT,
    "author" TEXT,
    "maintainer" TEXT,
    "depiction" TEXT,
    "nativeDepiction" TEXT,
    "sileoDepiction" TEXT,
    "header" TEXT,
    "tintColor" TEXT,
    "icon" TEXT,
    "section" TEXT,
    "tags" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "installedSize" INTEGER,

    CONSTRAINT "Package_pkey" PRIMARY KEY ("uuid")
);

-- CreateTable
CREATE TABLE "Cache" (
    "uuid" TEXT NOT NULL,
    "fileHash" TEXT NOT NULL,
    "repositorySlug" TEXT NOT NULL,

    CONSTRAINT "Cache_pkey" PRIMARY KEY ("uuid")
);

-- CreateTable
CREATE TABLE "Manifest" (
    "uuid" TEXT NOT NULL,
    "url" TEXT NOT NULL,
    "type" "ManifestType" NOT NULL,

    CONSTRAINT "Manifest_pkey" PRIMARY KEY ("uuid")
);

-- CreateIndex
CREATE UNIQUE INDEX "Repository_slug_key" ON "Repository"("slug");

-- CreateIndex
CREATE UNIQUE INDEX "Repository_originId_key" ON "Repository"("originId");

-- CreateIndex
CREATE INDEX "Repository_isPruned_idx" ON "Repository"("isPruned");

-- CreateIndex
CREATE UNIQUE INDEX "Origin_uuid_key" ON "Origin"("uuid");

-- CreateIndex
CREATE UNIQUE INDEX "Package_uuid_key" ON "Package"("uuid");

-- CreateIndex
CREATE INDEX "Package_isCurrent_isPruned_repositoryTier_idx" ON "Package"("isCurrent", "isPruned", "repositoryTier");

-- CreateIndex
CREATE UNIQUE INDEX "Cache_uuid_key" ON "Cache"("uuid");

-- CreateIndex
CREATE UNIQUE INDEX "Manifest_uuid_key" ON "Manifest"("uuid");

-- CreateIndex
CREATE INDEX "Manifest_url_type_idx" ON "Manifest"("url", "type");

-- AddForeignKey
ALTER TABLE "Repository" ADD CONSTRAINT "Repository_originId_fkey" FOREIGN KEY ("originId") REFERENCES "Origin"("uuid") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Package" ADD CONSTRAINT "Package_repositorySlug_fkey" FOREIGN KEY ("repositorySlug") REFERENCES "Repository"("slug") ON DELETE RESTRICT ON UPDATE CASCADE;
