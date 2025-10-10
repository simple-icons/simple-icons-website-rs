Feature: Change SVG path by manual input
	As a user
	I want to change the SVG path by typing a path in the path input
	So that I can quickly see the preview with a different SVG path

	Background:
		Given I see the preview generator page
		Then The brand input value is "Simple Icons"
		Then The path input value starts with "M12 0"
		Then The SVG paths of the preview start with "M12 0"

	Scenario: Change SVG path by manual input
		When I type "M0 0h24v24h-24z" in the path input
		Then The SVG paths of the preview are "M0 0h24v24h-24z"
