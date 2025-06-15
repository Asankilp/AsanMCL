export function convertToCompactUUID(uuid: string) {
    return uuid.replace(/-/g, "");
}