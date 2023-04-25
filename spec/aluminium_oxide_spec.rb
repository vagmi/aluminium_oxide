# frozen_string_literal: true

RSpec.describe AluminiumOxide do
  it "has a version number" do
    expect(AluminiumOxide::VERSION).not_to be nil
  end

  it "does something useful" do
    msg = AluminiumOxide.hello("world")
    expect(msg).to eq("Hello from Rust, world!")
  end

  describe AluminiumOxide::Greeter do
    before do
      @greeter = AluminiumOxide::Greeter.new("railsconf")
    end
    it "should greet with the proper message" do
      msg = @greeter.greet
      expect(msg).to eq("Hello, railsconf!")
    end
  end

  describe "AluminiumOxide::GREETER" do
    it "should have a constant greeter exposed" do
      msg = AluminiumOxide::GREETER.greet
      expect(msg).to eq("Hello, From Rust!")
    end
  end
end
