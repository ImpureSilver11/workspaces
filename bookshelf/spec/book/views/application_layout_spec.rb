require "spec_helper"

RSpec.describe Book::Views::ApplicationLayout, type: :view do
  let(:layout)   { Book::Views::ApplicationLayout.new({ format: :html }, "contents") }
  let(:rendered) { layout.render }

  it 'contains application name' do
    expect(rendered).to include('Book')
  end
end
