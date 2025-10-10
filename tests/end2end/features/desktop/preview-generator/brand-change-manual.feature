Feature: Change brand title by manual input
	As a user
	I want to change the brand by typing a title in the brand input
	So that I can quickly see the preview with a different brand title

	Background:
		Given I see the preview generator page
		Then The brand input value is "Simple Icons"
		Then The title in the preview is "Simple Icons Preview"
		Then The filename in the preview is "simpleicons.svg"
		Then The brand in the preview is "Brand: Simple Icons"

	Scenario: Change brand title by manual input
		When I type "Foo Bar" in the brand input
		Then The title in the preview is "Foo Bar Preview"
		Then The filename in the preview is "foobar.svg"
		Then The brand in the preview is "Brand: Foo Bar"
