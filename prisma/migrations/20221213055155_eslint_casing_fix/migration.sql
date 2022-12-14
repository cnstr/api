/*
  Warnings:

  - You are about to drop the column `hasReleaseGPG` on the `Origin` table. All the data in the column will be lost.
  - You are about to drop the column `usesHTTPS` on the `Origin` table. All the data in the column will be lost.
  - Added the required column `hasReleaseGpg` to the `Origin` table without a default value. This is not possible if the table is not empty.
  - Added the required column `usesHttps` to the `Origin` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Origin" DROP COLUMN "hasReleaseGPG",
DROP COLUMN "usesHTTPS",
ADD COLUMN     "hasReleaseGpg" BOOLEAN NOT NULL,
ADD COLUMN     "usesHttps" BOOLEAN NOT NULL;
