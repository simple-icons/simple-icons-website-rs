Feature: Download icon as SVG
	As a user
	I want to download the icon as a SVG file
	So that I can see the source SVG of the icon locally

	Background:
		Given I see the preview generator page
		Then the brand input value is "Simple Icons"

	Scenario: Download default icon as a SVG file
		When I click on the element "#preview-download-svg-button"
		Then a file named "simpleicons.svg" is downloaded within 3 seconds

	Scenario: Download icon with custom name changes filename of downloaded SVG file
		When When I type "Foo Bar" in the brand input
		And I click on the element "#preview-download-svg-button"
		Then a file named "foobar.svg" is downloaded within 3 seconds

	Scenario: Click "Download SVG" button input by pressing Ctrl + ⇩ keyboard shortcut
		When I press the "Ctrl" + "ArrowDown" keys, the event "onclick" is executed on the element "#preview-download-svg-button"
