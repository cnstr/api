/*
  Warnings:

  - You are about to drop the column `type` on the `Manifest` table. All the data in the column will be lost.
  - Added the required column `variant` to the `Manifest` table without a default value. This is not possible if the table is not empty.

*/
-- DropIndex
DROP INDEX "Manifest_url_type_idx";

-- AlterTable
ALTER TABLE "Manifest" DROP COLUMN "type",
ADD COLUMN     "variant" "ManifestType" NOT NULL;

-- CreateIndex
CREATE INDEX "Manifest_url_variant_idx" ON "Manifest"("url", "variant");
