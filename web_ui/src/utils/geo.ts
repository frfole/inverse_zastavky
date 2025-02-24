export function approx_distance(first: number[], second: number[]) {
    const deg2rad = Math.PI / 180
    const earth_radius = 6371
    const lat_diff = second[0] - first[0]
    const lon_diff = second[1] - first[1]
    return 2 * earth_radius * Math.asin(Math.sqrt((
        1 - Math.cos(lat_diff * deg2rad) + Math.cos(first[0] * deg2rad) * Math.cos(second[0] * deg2rad) * (1 - Math.cos(lon_diff * deg2rad))
    ) / 2))
}