Feature: Download preview
	As a user
	I want to download the preview as a PNG file
	So that I can upload it to a pull request in Simple Icons repository

	Background:
		Given I see the preview generator page
		Then the brand input value is "Simple Icons"

	Scenario: Download default preview as a PNG file
		When I click on the element "#preview-save-button"
		Then a file named "simpleicons.png" is downloaded within 3 seconds

	Scenario: Download icon with custom name changes filename of downloaded PNG file
		When When I type "Foo Bar" in the brand input
		And I click on the element "#preview-save-button"
		Then a file named "foobar.png" is downloaded within 3 seconds

	Scenario: Click "Save Preview" button input by pressing Ctrl + S keyboard shortcut
		When I press the "Ctrl" + "S" keys, the event "onclick" is executed on the element "#preview-save-button"
