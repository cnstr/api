/*
  Warnings:

  - A unique constraint covering the columns `[repositorySlug]` on the table `Cache` will be added. If there are existing duplicate values, this will fail.

*/
-- CreateIndex
CREATE UNIQUE INDEX "Cache_repositorySlug_key" ON "Cache"("repositorySlug");
