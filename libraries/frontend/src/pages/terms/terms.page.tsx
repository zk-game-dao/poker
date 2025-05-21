import { Fragment, memo } from 'react';

import { LayoutComponent } from '@zk-game-dao/ui';

const TERMS = [
  {
    title: "Introduction",
    content:
      "An overview of the purpose of the document and the parties involved.",
  },
  {
    title: "Definitions",
    content:
      "Clarification of key terms used throughout the document to avoid misunderstandings.",
  },
  {
    title: "Acceptance of Terms",
    content:
      "Information on how users agree to the terms, often by using the service or product.",
  },
  {
    title: "User Responsibilities",
    content:
      "Specific requirements and behaviors expected from users, such as compliance with laws and respectful conduct.",
  },
  {
    title: "Account Terms",
    content: "Rules regarding account creation, security, and management.",
  },
  {
    title: "Usage Restrictions",
    content:
      "Limitations on how the service or product can be used, including prohibited activities.",
  },
  {
    title: "Intellectual Property",
    content:
      "Information about the ownership of content, trademarks, and patents.",
  },
  {
    title: "Privacy Policy",
    content: "Details on how user data is collected, used, and protected.",
  },
  {
    title: "Payment Terms",
    content:
      "Information about pricing, billing, and payment methods, if applicable.",
  },
  {
    title: "Termination",
    content:
      "Conditions under which the agreement can be terminated by either party.",
  },
  {
    title: "Disclaimers and Limitation of Liability",
    content:
      "Statements that limit the service provider's legal liability for issues like data loss or service outages.",
  },
  {
    title: "Governing Law",
    content:
      "Specification of the legal jurisdiction that will govern the terms and conditions.",
  },
  {
    title: "Dispute Resolution",
    content:
      "Processes for resolving disagreements, such as arbitration or mediation.",
  },
  {
    title: "Amendments",
    content:
      "Information on how changes to the terms will be communicated and implemented.",
  },
  {
    title: "Contact Information",
    content:
      "Details on how users can get in touch with the service provider for support or questions.",
  },
  {
    title: "Introduction",
    content:
      "An overview of the purpose of the document and the parties involved.",
  },
];

export const TermsPage = memo(() => (
  <LayoutComponent
    footer
    hero={{
      title: "Terms & Conditions",
      subTitle:
        "Our set of rules and guidelines that outline the rights, responsibilities, and limitations for both ZKPoker and its users.",
    }}
    className="pb-48"
  >
    {TERMS.map(({ title, content }) => (
      <Fragment key={title}>
        <h2 className="text-material-heavy-1 type-top max-w-[650px] mx-auto mt-8">
          {title}
        </h2>
        <p className="text-material-heavy-1 mt-3 max-w-[650px] mx-auto">
          {content}
        </p>
      </Fragment>
    ))}
  </LayoutComponent>
));
TermsPage.displayName = "TermsPage";