use sigmaos_buildbtw::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_buildbtw::Buildbtw::new("sigmaos-buildbtw").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-buildbtw"));
}
