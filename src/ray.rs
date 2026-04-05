use super::geometry::Vector3;

#[cfg(test)]
mod tests;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vector3<f64> {
        // TODO: how to get rid of clone???
        self.origin + self.direction * t
    }
}
