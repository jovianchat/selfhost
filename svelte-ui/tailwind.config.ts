import type { Config } from 'tailwindcss';
import typography from '@tailwindcss/typography';
import daisyui from 'daisyui';
import daisyTheme from 'daisyui/src/theming/themes';

const config: Config = {
	content: ['./src/routes/**/*.{html,js,svelte,ts}', './src/components/**/*.{html,js,svelte,ts}'],
	theme: {
		// extend: {
		//   backgroundImage: {
		//     "gradient-radial": "radial-gradient(var(--tw-gradient-stops))",
		//     "gradient-conic":
		//       "conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))",
		//   },
		// },
	},
	plugins: [typography, daisyui],
	daisyui: {
		themes: [
			{
				dark: {
					...daisyTheme.dark,
					// "primary": "#e95f0b",
					// "base-100": '#2a2a24',
					// "base-300": '#20201d',
					'base-100': '#262622',
					'base-200': '#1d232a',
					'base-300': '#1c1c1c',
					accent: '#34d399',
					// "primary": '#ff5722',
					// "cream": '#f2f2f2',
					// https://palettes.shecodes.io/palettes/633#palette
					'base-content': '#e6e6e6',
					error: '#ff0000',
					'error-content': '#ffffff',
					'--rounded-box': '0.5rem', // border radius rounded-box utility class, used in card and other large boxes
					'--rounded-btn': '0.5rem' // border radius rounded-btn utility class, used in buttons and similar element
				}
			}
		]
	}
};
export default config;
