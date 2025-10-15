Feature: Upload SVG file
	As a user
	I want to change the path of the icon by uploading an SVG file
	So that I can quickly see the preview of a different icon

	Background:
		Given I see the preview generator page
		Then the brand input value is "Simple Icons"

	Scenario: Change preview icon by uploading a file in the "Upload SVG" input
		When I upload the file "node_modules/simple-icons/icons/leptos.svg" in the "#preview-upload-svg-button" input
		Then the brand input value is "Leptos"
		Then the title in the preview is "Leptos Preview"
		Then the filename in the preview is "leptos.svg"
		Then the brand in the preview is "Brand: Leptos"
		Then the path input value starts with "M10.097 17.876"
		Then the SVG paths of the preview start with "M10.097 17.876"
		Then the logo SVG paths of the badges in the preview start with "M10.097 17.876"
		Then the color input value is "EF3939"
		Then the color in the preview is "Color: #EF3939"
		Then the background color of the preview is #EF3939
		Then the color of the badges in the preview is #EF3939

	Scenario: Click "Upload SVG" button file input by pressing Ctrl + ⇧ keyboard shortcut
		When I press the "Ctrl" + "ArrowUp" keys, the event "onclick" is executed on the element "#preview-upload-svg-button"

	Scenario: Click "Upload SVG" triggers a click on the hidden file input
		When I click on the element "button[title='Upload SVG']", the event "onclick" is executed on the element "#preview-upload-svg-button"
