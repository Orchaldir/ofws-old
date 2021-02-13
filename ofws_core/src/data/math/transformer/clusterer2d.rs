use crate::data::math::size2d::Size2d;
use serde::{Deserialize, Serialize};

/// Determines a cluster id from both inputs. E.g. biome from rainfall & temperature.
#[derive(Debug, Serialize, Deserialize)]
pub struct Clusterer2d {
    lookup_table_size: Size2d,
    cluster_size: Size2d,
    cluster_id_lookup: Vec<u8>,
}

impl Clusterer2d {
    /// Returns a clusterer, if valid:
    ///
    /// ```
    ///# use ofws_core::data::math::size2d::Size2d;
    ///# use ofws_core::data::math::transformer::clusterer2d::Clusterer2d;
    /// assert!(Clusterer2d::new(Size2d::new(2,  2), vec![10, 20, 30, 40, 50, 60]).is_err());
    /// assert!(Clusterer2d::new(Size2d::new(2, 10), vec![10, 20, 30, 40, 50, 60]).is_err());
    /// assert!(Clusterer2d::new(Size2d::new(0,  0), vec![10, 20, 30, 40, 50, 60]).is_err());
    /// assert!(Clusterer2d::new(Size2d::new(0,  0), vec![]).is_err());
    /// ```
    pub fn new(size: Size2d, cluster_id_lookup: Vec<u8>) -> Result<Clusterer2d, &'static str> {
        if size.get_area() != cluster_id_lookup.len() {
            return Err("Size & look up table don't match!");
        } else if cluster_id_lookup.len() < 2 {
            return Err("Needs more than 1 cluster!");
        }

        let width = calculate_cluster_size(size.width());
        let height = calculate_cluster_size(size.height());

        Ok(Clusterer2d {
            lookup_table_size: size,
            cluster_size: Size2d::new(width, height),
            cluster_id_lookup,
        })
    }

    /// Calculates the cluster of 2 inputs.
    ///
    /// ```
    ///# use ofws_core::data::math::size2d::Size2d;
    ///# use ofws_core::data::math::transformer::clusterer2d::Clusterer2d;
    /// let clusterer = Clusterer2d::new(Size2d::new(3, 2), vec![10, 20, 30, 40, 50, 60]).unwrap();
    ///
    /// assert_eq!(clusterer.cluster(  0,   0), 10);
    /// assert_eq!(clusterer.cluster(100,  60), 20);
    /// assert_eq!(clusterer.cluster(200, 100), 30);
    /// assert_eq!(clusterer.cluster( 60, 170), 40);
    /// assert_eq!(clusterer.cluster(170, 200), 50);
    /// assert_eq!(clusterer.cluster(255, 255), 60);
    /// ```
    pub fn cluster(&self, input0: u8, input1: u8) -> u8 {
        let x = input0 as u32 / self.cluster_size.width();
        let y = input1 as u32 / self.cluster_size.height();
        let index = self.lookup_table_size.to_index(x, y);

        *self.cluster_id_lookup.get(index).unwrap_or_else(|| {
            panic!(
                "Index {} is too large for {} clusters!",
                index,
                self.cluster_id_lookup.len()
            )
        })
    }
}

fn calculate_cluster_size(number_of_clusters: u32) -> u32 {
    (256.0 / number_of_clusters as f32).ceil() as u32
}
