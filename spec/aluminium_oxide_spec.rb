# frozen_string_literal: true

RSpec.describe AluminiumOxide do
  it "has a version number" do
    expect(AluminiumOxide::VERSION).not_to be nil
  end

  it "does something useful" do
    msg = AluminiumOxide.hello("world")
    expect(msg).to eq("Hello from Rust, world!")
  end
end
