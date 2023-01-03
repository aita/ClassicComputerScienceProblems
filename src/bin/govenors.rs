use classic_computer_science_problems::kmeans;

#[derive(Debug)]
struct Govenor<'a>(f64, f64, &'a str);

fn main() {
    let governors = vec![
        Govenor(-86.79113, 72.0, "Alabama"),
        Govenor(-152.404419, 66.0, "Alaska"),
        Govenor(-111.431221, 53.0, "Arizona"),
        Govenor(-92.373123, 66.0, "Arkansas"),
        Govenor(-119.681564, 79.0, "California"),
        Govenor(-105.311104, 65.0, "Colorado"),
        Govenor(-72.755371, 61.0, "Connecticut"),
        Govenor(-75.507141, 61.0, "Delaware"),
        Govenor(-81.686783, 64.0, "Florida"),
        Govenor(-83.643074, 74.0, "Georgia"),
        Govenor(-157.498337, 60.0, "Hawaii"),
        Govenor(-114.478828, 75.0, "Idaho"),
        Govenor(-88.986137, 60.0, "Illinois"),
        Govenor(-86.258278, 49.0, "Indiana"),
        Govenor(-93.210526, 57.0, "Iowa"),
        Govenor(-96.726486, 60.0, "Kansas"),
        Govenor(-84.670067, 50.0, "Kentucky"),
        Govenor(-91.867805, 50.0, "Louisiana"),
        Govenor(-69.381927, 68.0, "Maine"),
        Govenor(-76.802101, 61.0, "Maryland"),
        Govenor(-71.530106, 60.0, "Massachusetts"),
        Govenor(-84.536095, 58.0, "Michigan"),
        Govenor(-93.900192, 70.0, "Minnesota"),
        Govenor(-89.678696, 62.0, "Mississippi"),
        Govenor(-92.288368, 43.0, "Missouri"),
        Govenor(-110.454353, 51.0, "Montana"),
        Govenor(-98.268082, 52.0, "Nebraska"),
        Govenor(-117.055374, 53.0, "Nevada"),
        Govenor(-71.563896, 42.0, "New Hampshire"),
        Govenor(-74.521011, 54.0, "New Jersey"),
        Govenor(-106.248482, 57.0, "New Mexico"),
        Govenor(-74.948051, 59.0, "New York"),
        Govenor(-79.806419, 60.0, "North Carolina"),
        Govenor(-99.784012, 60.0, "North Dakota"),
        Govenor(-82.764915, 65.0, "Ohio"),
        Govenor(-96.928917, 62.0, "Oklahoma"),
        Govenor(-122.070938, 56.0, "Oregon"),
        Govenor(-77.209755, 68.0, "Pennsylvania"),
        Govenor(-71.51178, 46.0, "Rhode Island"),
        Govenor(-80.945007, 70.0, "South Carolina"),
        Govenor(-99.438828, 64.0, "South Dakota"),
        Govenor(-86.692345, 58.0, "Tennessee"),
        Govenor(-97.563461, 59.0, "Texas"),
        Govenor(-111.862434, 70.0, "Utah"),
        Govenor(-72.710686, 58.0, "Vermont"),
        Govenor(-78.169968, 60.0, "Virginia"),
        Govenor(-121.490494, 66.0, "Washington"),
        Govenor(-80.954453, 66.0, "West Virginia"),
        Govenor(-89.616508, 49.0, "Wisconsin"),
        Govenor(-107.30249, 55.0, "Wyoming"),
    ];
    let mut kmeans = kmeans::KMeans::new(
        2,
        governors
            .iter()
            .map(|g| kmeans::DataPoint::new(vec![g.0, g.1]))
            .collect(),
    );
    let clusters = kmeans.run(100);
    for (i, cluster) in clusters.iter().enumerate() {
        let cluster_governors = cluster
            .points()
            .iter()
            .map(|i| &governors[*i])
            .collect::<Vec<_>>();
        println!("Cluster {}: {:?}", i, cluster_governors);
    }
}
