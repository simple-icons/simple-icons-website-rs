Feature: Change SVG path by manual input
	As a user
	I want to change the SVG path by typing a path in the path input
	So that I can quickly see the preview with a different SVG path

	Background:
		Given I see the preview generator page
		Then the brand input value is "Simple Icons"
		Then the path input value starts with "M12 0"
		Then the SVG paths of the preview start with "M12 0"

	Scenario: Change SVG path by manual input
		When I type "M0 0h24v24h-24z" in the path input
		Then the SVG paths of the preview are "M0 0h24v24h-24z"
		Then the logo SVG paths of the badges in the preview are "M0 0h24v24h-24z"
